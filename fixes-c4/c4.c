#include <stdio.h> //lib containing features that include file handling, console i/o, buffer management, error handling
                   //performs geenral input/output operations in c
#include <stdlib.h>//lib containing functions for meamory management, process control, conversions (atoi,atof, etc), sorting and searching, random num generation, math operations
                   //set of general purpose functions in c
#include <memory.h>//found in string.h header, used for memory manipulation of blocks
#include <unistd.h>//mainly used to perform low level system operstions
#include <fcntl.h>//includes operators that specify: file descriptor manipulation, file opening flags, file locking
#define int long long

char *p, *lp, // current position in source code
     *data;   // data/bss pointer

int *e, *le,  // current position in emitted code
    *id,      // currently parsed identifier
    *sym,     // symbol table (simple list of identifiers)
    tk,       // current token
    ival,     // current token value
    ty,       // current expression type
    loc,      // local variable offset
    line,     // current line number
    src,      // print source and assembly flag
    debug;    // print executed instructions

// tokens and classes (operators last and in precedence order)
enum {
  Num = 128, Fun, Sys, Glo, Loc, Id,
  Char, Else, Enum, If, Int, Return, Sizeof, While,
  Assign, Cond, Lor, Lan, Or, Xor, And, Eq, Ne, Lt, Gt, Le, Ge, Shl, Shr, Add, Sub, Mul, Div, Mod, Inc, Dec, Brak
};

// opcodes
enum { LEA ,IMM ,JMP ,JSR ,BZ  ,BNZ ,ENT ,ADJ ,LEV ,LI  ,LC  ,SI  ,SC  ,PSH ,
       OR  ,XOR ,AND ,EQ  ,NE  ,LT  ,GT  ,LE  ,GE  ,SHL ,SHR ,ADD ,SUB ,MUL ,DIV ,MOD ,
       OPEN,READ,CLOS,PRTF,MALC,FREE,MSET,MCMP,EXIT };

// types
enum { CHAR, INT, PTR };

// identifier offsets (since we can't create an ident struct)
enum { Tk, Hash, Name, Class, Type, Val, HClass, HType, HVal, Idsz };

void next() // Responsible for advancing to the next token in the input
{
  char *pp; // Temporary pointer for storing string positions

  // Loop to continuously process the input until a token is identified
  while (tk = *p) {
    ++p; // Move to the next character
    
    // Handle newlines: Print debugging output if 'src' is set
    if (tk == '\n') {
      if (src) {
        printf("%d: %.*s", line, p - lp, lp); // Print the current line
        lp = p; // Update lp to the new line start
        
        // Loop to process token list and print them
        while (le < e) {
          printf("%8.4s", &"LEA ,IMM ,JMP ,JSR ,BZ  ,BNZ ,ENT ,ADJ ,LEV ,LI  ,LC  ,SI  ,SC  ,PSH ,"
                           "OR  ,XOR ,AND ,EQ  ,NE  ,LT  ,GT  ,LE  ,GE  ,SHL ,SHR ,ADD ,SUB ,MUL ,DIV ,MOD ,"
                           "OPEN,READ,CLOS,PRTF,MALC,FREE,MSET,MCMP,EXIT,"[*++le * 5]);
          if (*le <= ADJ) printf(" %d\n", *++le); else printf("\n");
        }
      }
      ++line; // Increase line count
    }
    
    // Handle preprocessor directives (skip until newline)
    else if (tk == '#') {
      while (*p != 0 && *p != '\n') ++p;
    }
    
    // Process identifiers (letters, numbers, and underscores)
    else if ((tk >= 'a' && tk <= 'z') || (tk >= 'A' && tk <= 'Z') || tk == '_') {
      pp = p - 1; // Store beginning of identifier
      
      // Continue reading identifier characters
      while ((*p >= 'a' && *p <= 'z') || (*p >= 'A' && *p <= 'Z') || (*p >= '0' && *p <= '9') || *p == '_')
        tk = tk * 147 + *p++; // Hash function for identifiers
      
      tk = (tk << 6) + (p - pp); // Additional hashing to differentiate similar names
      id = sym; // Start scanning the symbol table
      
      // Check if identifier exists in symbol table
      while (id[Tk]) {
        if (tk == id[Hash] && !memcmp((char *)id[Name], pp, p - pp)) { 
          tk = id[Tk]; 
          return; // Identifier found, return
        }
        id = id + Idsz; // Move to next symbol table entry
      }
      
      // If identifier is new, add to symbol table
      id[Name] = (int)pp;
      id[Hash] = tk;
      tk = id[Tk] = Id;
      return;
    }
    
    // Process numbers (integers and hexadecimal values)
    else if (tk >= '0' && tk <= '9') {
      if (ival = tk - '0') {
        while (*p >= '0' && *p <= '9') ival = ival * 10 + *p++ - '0'; // Decimal numbers
      }
      else if (*p == 'x' || *p == 'X') { // Hexadecimal numbers
        while ((tk = *++p) && ((tk >= '0' && tk <= '9') || (tk >= 'a' && tk <= 'f') || (tk >= 'A' && tk <= 'F')))
          ival = ival * 16 + (tk & 15) + (tk >= 'A' ? 9 : 0);
      }
      else { // Octal numbers
        while (*p >= '0' && *p <= '7') ival = ival * 8 + *p++ - '0';
      }
      tk = Num;
      return;
    }
    
    // Handle comments and division
    else if (tk == '/') {
      if (*p == '/') { // Single-line comment
        ++p;
        while (*p != 0 && *p != '\n') ++p;
      }
      else { // Division operator
        tk = Div;
        return;
      }
    }
    
    // Process string and character literals
    else if (tk == '\'' || tk == '"') {
      pp = data;
      while (*p != 0 && *p != tk) {
        if ((ival = *p++) == '\\') { // Handle escape sequences
          if ((ival = *p++) == 'n') ival = '\n';
        }
        if (tk == '"') *data++ = ival;
      }
      ++p;
      if (tk == '"') ival = (int)pp; else tk = Num;
      return;
    }
    
    // Handle operators and delimiters
    else if (tk == '=') { if (*p == '=') { ++p; tk = Eq; } else tk = Assign; return; }
    else if (tk == '+') { if (*p == '+') { ++p; tk = Inc; } else tk = Add; return; }
    else if (tk == '-') { if (*p == '-') { ++p; tk = Dec; } else tk = Sub; return; }
    else if (tk == '!') { if (*p == '=') { ++p; tk = Ne; } return; }
    else if (tk == '<') { if (*p == '=') { ++p; tk = Le; } else if (*p == '<') { ++p; tk = Shl; } else tk = Lt; return; }
    else if (tk == '>') { if (*p == '=') { ++p; tk = Ge; } else if (*p == '>') { ++p; tk = Shr; } else tk = Gt; return; }
    else if (tk == '|') { if (*p == '|') { ++p; tk = Lor; } else tk = Or; return; }
    else if (tk == '&') { if (*p == '&') { ++p; tk = Lan; } else tk = And; return; }
    else if (tk == '^') { tk = Xor; return; }
    else if (tk == '%') { tk = Mod; return; }
    else if (tk == '*') { tk = Mul; return; }
    else if (tk == '[') { tk = Brak; return; }
    else if (tk == '?') { tk = Cond; return; }
    else if (tk == '~' || tk == ';' || tk == '{' || tk == '}' || tk == '(' || tk == ')' || tk == ']' || tk == ',' || tk == ':') return;
  }
}


void expr(int lev) // Evaluates expressions based on the current token
// Handles operations and applies operator precedence
// Manages type checks and type conversions
{
  int t, *d;

  if (!tk) { 
    printf("%d: unexpected eof in expression\n", line); 
    exit(-1); 
  }
  
  // Handle numeric literals
  else if (tk == Num) { 
    *++e = IMM; 
    *++e = ival; 
    next(); 
    ty = INT; 
  }
  
  // Handle string literals
  else if (tk == '"') {
    *++e = IMM; 
    *++e = ival; 
    next();
    while (tk == '"') next();
    data = (char *)((int)data + sizeof(int) & -sizeof(int)); 
    ty = PTR; 
  }
  
  // Handle sizeof operator
  else if (tk == Sizeof) {
    next();
    if (tk == '(') next(); 
    else { 
      printf("%d: open paren expected in sizeof\n", line); 
      exit(-1); 
    }
    ty = INT; 
    if (tk == Int) next(); 
    else if (tk == Char) { 
      next(); 
      ty = CHAR; 
    }
    while (tk == Mul) { 
      next(); 
      ty = ty + PTR; 
    }
    if (tk == ')') next(); 
    else { 
      printf("%d: close paren expected in sizeof\n", line); 
      exit(-1); 
    }
    *++e = IMM; 
    *++e = (ty == CHAR) ? sizeof(char) : sizeof(int);
    ty = INT;
  }
  
  // Handle identifiers
  else if (tk == Id) {
    d = id; 
    next();
    if (tk == '(') { // Function call
      next();
      t = 0;
      while (tk != ')') { 
        expr(Assign); 
        *++e = PSH; 
        ++t; 
        if (tk == ',') next(); 
      }
      next();
      if (d[Class] == Sys) *++e = d[Val];
      else if (d[Class] == Fun) { 
        *++e = JSR; 
        *++e = d[Val]; 
      }
      else { 
        printf("%d: bad function call\n", line); 
        exit(-1); 
      }
      if (t) { 
        *++e = ADJ; 
        *++e = t; 
      }
      ty = d[Type];
    }
    else if (d[Class] == Num) { 
      *++e = IMM; 
      *++e = d[Val]; 
      ty = INT; 
    }
    else { // Variable lookup
      if (d[Class] == Loc) { 
        *++e = LEA; 
        *++e = loc - d[Val]; 
      }
      else if (d[Class] == Glo) { 
        *++e = IMM; 
        *++e = d[Val]; 
      }
      else { 
        printf("%d: undefined variable\n", line); 
        exit(-1); 
      }
      *++e = ((ty = d[Type]) == CHAR) ? LC : LI;
    }
  }
  
  // Handle parentheses
  else if (tk == '(') {
    next();
    if (tk == Int || tk == Char) { // Type casting
      t = (tk == Int) ? INT : CHAR; 
      next();
      while (tk == Mul) { 
        next(); 
        t = t + PTR; 
      }
      if (tk == ')') next(); 
      else { 
        printf("%d: bad cast\n", line); 
        exit(-1); 
      }
      expr(Inc);
      ty = t;
    }
    else { // Regular expression in parentheses
      expr(Assign);
      if (tk == ')') next(); 
      else { 
        printf("%d: close paren expected\n", line); 
        exit(-1); 
      }
    }
  }
  
  // Handle dereference (pointer access)
  else if (tk == Mul) {
    next(); 
    expr(Inc);
    if (ty > INT) ty = ty - PTR; 
    else { 
      printf("%d: bad dereference\n", line); 
      exit(-1); 
    }
    *++e = (ty == CHAR) ? LC : LI;
  }
  
  // Handle address-of (&) operator
  else if (tk == And) {
    next(); 
    expr(Inc);
    if (*e == LC || *e == LI) --e; 
    else { 
      printf("%d: bad address-of\n", line); 
      exit(-1); 
    }
    ty = ty + PTR;
  }
  
  // Handle logical NOT (!)
  else if (tk == '!') { 
    next(); 
    expr(Inc); 
    *++e = PSH; 
    *++e = IMM; 
    *++e = 0; 
    *++e = EQ; 
    ty = INT; 
  }
  
  // Handle bitwise NOT (~)
  else if (tk == '~') { 
    next(); 
    expr(Inc); 
    *++e = PSH; 
    *++e = IMM; 
    *++e = -1; 
    *++e = XOR; 
    ty = INT; 
  }
  
  // Handle addition
  else if (tk == Add) { 
    next(); 
    expr(Inc); 
    ty = INT; 
  }
  
  // Handle subtraction (negative numbers)
  else if (tk == Sub) {
    next(); 
    *++e = IMM;
    if (tk == Num) { 
      *++e = -ival; 
      next(); 
    } else { 
      *++e = -1; 
      *++e = PSH; 
      expr(Inc); 
      *++e = MUL; 
    }
    ty = INT;
  }
  
  else { 
    printf("%d: bad expression\n", line); 
    exit(-1); 
  }
}

void stmt() // Handles high-level constructs like if-statements and loops
// Modifies code emission pointers to compile these constructs
// Code emission pointers manage the location of code generation in memory
{
  int *a, *b;

  // Handle if-else statements
  if (tk == If) {
    next();
    if (tk == '(') next(); 
    else { printf("%d: open paren expected\n", line); exit(-1); }
    expr(Assign);
    if (tk == ')') next(); 
    else { printf("%d: close paren expected\n", line); exit(-1); }
    
    *++e = BZ; // Generate branch-if-zero instruction
    b = ++e;   // Store jump location for later back-patching
    stmt();    // Process the "then" block
    
    if (tk == Else) { // Handle optional else block
      *b = (int)(e + 3); 
      *++e = JMP; // Jump over the else block
      b = ++e;
      next();
      stmt();
    }
    *b = (int)(e + 1); // Patch jump target
  }
  
  // Handle while loops
  else if (tk == While) {
    next();
    a = e + 1; // Mark the beginning of the loop
    if (tk == '(') next(); 
    else { printf("%d: open paren expected\n", line); exit(-1); }
    expr(Assign);
    if (tk == ')') next(); 
    else { printf("%d: close paren expected\n", line); exit(-1); }
    
    *++e = BZ; // Generate branch-if-zero instruction for loop condition
    b = ++e;
    stmt(); // Process loop body
    
    *++e = JMP; // Generate unconditional jump to loop start
    *++e = (int)a; 
    *b = (int)(e + 1); // Patch jump target
  }
  
  // Handle return statements
  else if (tk == Return) {
    next();
    if (tk != ';') expr(Assign); // Evaluate return expression if present
    *++e = LEV; // Generate return instruction
    if (tk == ';') next(); 
    else { printf("%d: semicolon expected\n", line); exit(-1); }
  }
  
  // Handle block statements { ... }
  else if (tk == '{') {
    next();
    while (tk != '}') stmt(); // Process all statements inside block
    next();
  }
  
  // Handle empty statements (single semicolon)
  else if (tk == ';') {
    next();
  }
  
  // Handle expression statements (assignments, function calls, etc.)
  else {
    expr(Assign);
    if (tk == ';') next(); 
    else { printf("%d: semicolon expected\n", line); exit(-1); }
  }
}
int main(int argc, char **argv) // Entry point of the program
// Handles command line args to set flags or load files
// Sets up memory for different segments (code, memory, stack)
// Enters a loop to interpret or compile input code
{
  int fd, bt, ty, poolsz, *idmain;
  int *pc, *sp, *bp, a, cycle; // VM registers
  int i, *t; // Temporary variables

  // Process command-line arguments
  --argc; ++argv;
  if (argc > 0 && **argv == '-' && (*argv)[1] == 's') { src = 1; --argc; ++argv; } // Enable source debugging
  if (argc > 0 && **argv == '-' && (*argv)[1] == 'd') { debug = 1; --argc; ++argv; } // Enable debug mode
  if (argc < 1) { printf("usage: c4 [-s] [-d] file ...\n"); return -1; } // Ensure input file is provided

  // Open the input source file
  if ((fd = open(*argv, 0)) < 0) { printf("could not open(%s)\n", *argv); return -1; }

  // Allocate memory for different program segments
  poolsz = 256*1024; // Set memory pool size
  if (!(sym = malloc(poolsz))) { printf("could not malloc(%d) symbol area\n", poolsz); return -1; }
  if (!(le = e = malloc(poolsz))) { printf("could not malloc(%d) text area\n", poolsz); return -1; }
  if (!(data = malloc(poolsz))) { printf("could not malloc(%d) data area\n", poolsz); return -1; }
  if (!(sp = malloc(poolsz))) { printf("could not malloc(%d) stack area\n", poolsz); return -1; }

  // Initialize memory segments to zero
  memset(sym,  0, poolsz);
  memset(e,    0, poolsz);
  memset(data, 0, poolsz);

  // Populate symbol table with keywords and system functions
  p = "char else enum if int return sizeof while open read close printf malloc free memset memcmp exit void main";
  i = Char; while (i <= While) { next(); id[Tk] = i++; } // Add keywords
  i = OPEN; while (i <= EXIT) { next(); id[Class] = Sys; id[Type] = INT; id[Val] = i++; } // Add built-in system functions
  next(); id[Tk] = Char; // Handle "void" keyword
  next(); idmain = id; // Keep reference to main function

  // Read source file into memory
  if (!(lp = p = malloc(poolsz))) { printf("could not malloc(%d) source area\n", poolsz); return -1; }
  if ((i = read(fd, p, poolsz-1)) <= 0) { printf("read() returned %d\n", i); return -1; }
  p[i] = 0; // Null-terminate source buffer
  close(fd);

  // Start parsing the source file
  line = 1;
  next();
  while (tk) {
    bt = INT; // Default type is int
    if (tk == Int) next();
    else if (tk == Char) { next(); bt = CHAR; }
    else if (tk == Enum) { // Handle enum declarations
      next();
      if (tk != '{') next();
      if (tk == '{') {
        next();
        i = 0;
        while (tk != '}') {
          if (tk != Id) { printf("%d: bad enum identifier %d\n", line, tk); return -1; }
          next();
          if (tk == Assign) {
            next();
            if (tk != Num) { printf("%d: bad enum initializer\n", line); return -1; }
            i = ival;
            next();
          }
          id[Class] = Num; id[Type] = INT; id[Val] = i++;
          if (tk == ',') next();
        }
        next();
      }
    }
    
    // Handle global declarations
    while (tk != ';' && tk != '}') {
      ty = bt;
      while (tk == Mul) { next(); ty = ty + PTR; }
      if (tk != Id) { printf("%d: bad global declaration\n", line); return -1; }
      if (id[Class]) { printf("%d: duplicate global definition\n", line); return -1; }
      next();
      id[Type] = ty;
      
      if (tk == '(') { // Handle function definitions
        id[Class] = Fun;
        id[Val] = (int)(e + 1);
        next(); i = 0;
        while (tk != ')') {
          ty = INT;
          if (tk == Int) next();
          else if (tk == Char) { next(); ty = CHAR; }
          while (tk == Mul) { next(); ty = ty + PTR; }
          if (tk != Id) { printf("%d: bad parameter declaration\n", line); return -1; }
          id[HClass] = id[Class]; id[Class] = Loc;
          id[HType]  = id[Type];  id[Type] = ty;
          id[HVal]   = id[Val];   id[Val] = i++;
          next();
          if (tk == ',') next();
        }
        next();
        if (tk != '{') { printf("%d: bad function definition\n", line); return -1; }
        loc = ++i;
        next();
        while (tk == Int || tk == Char) {
          bt = (tk == Int) ? INT : CHAR;
          next();
          while (tk != ';') {
            ty = bt;
            while (tk == Mul) { next(); ty = ty + PTR; }
            if (tk != Id) { printf("%d: bad local declaration\n", line); return -1; }
            id[HClass] = id[Class]; id[Class] = Loc;
            id[HType]  = id[Type];  id[Type] = ty;
            id[HVal]   = id[Val];   id[Val] = ++i;
            next();
            if (tk == ',') next();
          }
          next();
        }
        *++e = ENT; *++e = i - loc;
        while (tk != '}') stmt();
        *++e = LEV;
      }
      else { // Global variable declaration
        id[Class] = Glo;
        id[Val] = (int)data;
        data = data + sizeof(int);
      }
      if (tk == ',') next();
    }
    next();
  }

  // Ensure main function is defined and setup stack for execution
  if (!(pc = (int *)idmain[Val])) { printf("main() not defined\n"); return -1; }
  if (src) return 0;
  bp = sp = (int *)((int)sp + poolsz);
  *--sp = EXIT;
  *--sp = PSH; t = sp;
  *--sp = argc;
  *--sp = (int)argv;
  *--sp = (int)t;

  // Execution loop for interpreting compiled code
  cycle = 0;
  while (1) {
    i = *pc++; ++cycle;
    if (i == EXIT) { printf("exit(%d) cycle = %d\n", *sp, cycle); return *sp; }
  }
}
