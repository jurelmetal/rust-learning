Programming
Java

Lesson 4

Loops

Contents

1. Loops ............................................................................. 3
do-while: a Loop with Postcondition  .  .  .  .  .  .  .  .  .  .  .  .  .  . . 4
for: Execute a Loop n Times   .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  . . 5
For-each  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  . . 6
The break and continue Operators  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  . . 7
The Return Operator   .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  . . 9

2. Main Provisions of Java Code Convention ................ 10

3. Working with the Eclipse integrated debugger ......... 15
Alternative Development Environments  .  .  .  .  .  .  .  .  .  . . 15
Eclipse Installation   .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  . . 16
Creating a Project in Eclipse  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  . . 16
Running the Java project in Eclipse   .  .  .  .  .  .  .  .  .  .  .  .  .  . . 18

2 1. Loops

1. Loops

Java has the while, do while, and for operators for imple-

menting looping repetitive algorithm .

The while Loop Operator .
Syntax:

while (expression) operation;

Expression in braces (loop condition) should be of boolean
type . If the loop condition is true, then the "operation" will be
executed, and the control will be passed back to the conditional
check of expression . If the value is false, then the loop body
execution will be terminated .
For example:

int i = 0;
while (i < 5) System.out.println(i++);

Outcome: 0, 1, 2, 3, 4, 5

Loop body is an operation of a block of code, which is
limited by the curly braces and located immediately following
the while operator .
For example:

int i = 0;
while (i < 5) { // beginning of the loop body
    i++;
    System.out.println(i);
} // end of the loop body

Outcome: 1, 2, 3, 4, 5

Note: If the loop condition is initially false, then the loop
body will never be fulfilled.

3

Lesson 4. Loops

For example:

int i = 0;
while (i > 5) {

    System.out.println(i++);
}
System.out.println(i);

Outcome: 0

If the members of the loop condition are not changed
within the loop body, and the loop condition is true, then this
loop will be executed infinitely .
Examples of an infinite loop:

while (true) {
    System.out.println("infinity");
}

do-while: a Loop with Postcondition

Syntax
do operation;
while(expression);
do {
operation1;
operation2;
}
while(expression);
The difference between the do while and the while loop is
that the body of the do while loop is executed once until the check
of the loop condition . Repeated execution of the loop body
will depend on the value of an expression, and if it is true, then
the program will return control to the beginning of the loop body .

4 1. Loops

For example:

int i = 0;
do
{
    i++;
    System.out.println(i);
} while (i > 10);

Outcome: 1

Syntax:

for: Execute a Loop n Times

for (<initialization>;

<condition>; <counter>)

operation;

The "initialization" block can contain declaration and
initialization of the variables that will be available in the loop
body . This block is optional .

The "condition" block should contain an expression,
the result of which should be a boolean value . This block is
optional .

The "counter" block can contain any operations . This

block is optional .

Example, in which all blocks are filled:

for (int i = 0; i < 10; i++)
{
    System.out.println(i);
}

5

Lesson 4. Loops

Example without the counter variable declaration
(it is required if you need access to the counter
beyond the loop body):

int i;
for (i = 0; i < 10; i++)
{
    System.out.println(i);
}

i+=2;
System.out.println(i);

Example with no initialization block:

int i = 0;

for (; i < 10; i++)
{
    System.out.println(i);
}

Example with multiple counters:

for (int i = 1, j = 3; i < 5 & j > 0; i++, --j)
{
    System.out.println("i = " + i + " j = " + j);
}

Outcome: i = 1 j = 3,     i = 2 j = 2,     i = 3 j = 1

For-each

Java has no specific keyword for iteration through ele-
ments of an array or a collection, which other languages have .
A special syntax of the for operator is used for implementing
the for-each loop .

6 1. Loops

Syntax:
for (type element : collection) {
    operation;
}

Example for an array:

int [] numbers = new int[] { 3, 2, 1 };
for (int number : numbers)
{
    System.out.print(number + " ");
}

Example for a collection:

List<Integer> lists = new ArrayList<Integer>();
lists.add(1);
lists.add(3);
for (Integer value: lists)
{
    System.out.println(value);
}

The break and continue Operators

The break operator terminates loop execution and passes

control to the end of the loop body .
For example:

int i = 0;
while (i < 10)
{
    System.out.println(i);
    if (i >= 2) break;
    i++;
}

Outcome: 0, 1

7

Lesson 4. Loops

The break operator can be used in conjunction with a la-
bel, for example, for simultaneous termination of multiple
nested loops . In this case, the control will be passed to the end
of the block with the label specified after the break operator .
Label is used for naming a block of code .

For example:

    two: for (int n = 0; n < 10; n++) {
       for (int j = 10; j > 0; j--) {
          System.out.print(j - n + " ");
             if (j + n == 5 && n > 0)
                break two;
      }
}

The continue operator terminates the loop body execu-
tion and passes control to the beginning of the loop . At that,
current loop iteration is skipped .
For example:

int i = 0;
while (i < 10)
{
    i++;
    if (i % 2 == 0)
    continue;
    System.out.println(i);
}

Outcome: 1, 3, 5, 7, 9

8 1. Loops

The continue operator can also be used with a label .

For example:

one: for (i = 0; i < 10; i++) {
       for (int j = 0; j < 10; j++) {
         if (i == j)
             System.out.printf("i=%d j=%d \n", i, j);
       }
     }

continue one;

The Return Operator

The return operator terminates execution of a method,

returning the control to the method call .

9

Lesson 4. Loops

2. Main Provisions of Java Code Convention
Code Conventions for the Java Programming Language
is  an  agreement  on  code  formatting .  Compliance  with
the provisions of the Convention makes your code more
readable and understandable to other programmers . These
provisions are recommendations based on personal experience
of programmers when working with the code .

Class Names

The files with the source code should have the java ex-

tension .

The bytecode files should have the class extension .

Source File Organization

Files longer than 2000 lines are cumbersome and should be
avoided . Files with a larger number of lines make it difficult to
find the right block of code, increase the time of compilation
of and loading a class to a java machine .

The file should be divided into sections that are separated

by an empty string .

A single file can contain only one class with the public

modifier (base class) .

It is not desirable to have other (non-public) classes in

a single file .

The order of sections in a class:

 ■ JavaDoc;
 ■ Class header;
 ■ Static fields;
 ■ Non-static fields;

10 2. Main Provisions of Java Code Convention

 ■ Constructors;
 ■ Methods .

Indentations

Four spaces should be used as a unit of indentation (tab-

ulation can be used) .

Lines should not be longer than 80 characters .

Naming of Classes and Interfaces

Class names should be capitalized, consisting of letters of
the Latin aphabet without spaces . Class name should corre-
spond to an object of this class .
For example:

class  Cats // bad
class  Cat // good

If the class name contains more than one word, then each
word should be capitalized (Camel notation) . Class name
should be corresponding .
For example:

class  TileIterator

Interface naming rules are the same as class naming rules .
It is not desirable to use additional indications of belonging
to an interface in the interface name .

interface IClonable // bad
interface Clonable // good

11

Lesson 4. Loops

If the class name contains an abbreviation of more than
two letters, then all the letters of this abbreviation, except
the first one, should be lowercase .
For example:

class  FPSRenderer // bad
class  FpsRenderer // good

Variable Naming

Multiple variables should be declared in different lines each .

int a, b; // bad
int a; // good
int b; // good

Variables should be initialized at declaration where possible .

int a;
...
a = 255;  // bad
int a = 255; // good

Variables should be declared as close as possible to where

they are used .

int a = 255; // bad
System.out.println("file")
a + = 127;
System.out.println("file")
int a = 255; // good
int с = a + 127;

12 2. Main Provisions of Java Code Convention

Arrays

When declaring an array, brackets should be specified

after the variable type, not the name .

int a [] = new int[3]; // bad
int[] a = new int[3]; // good

int [] a [] = new int[3]; // bad
int[][] a = new int[3][3]; // good

Constants

It is commonly accepted to name constants with all letters

capitalized, and to use underscore as a separator .
Examples:

final int MAX_STEP = 3;// good
final String DELIMITER = ";";// good

Methods

Method names should consist of lowercase Latin letters .
The first word in the method name should be a verb of an
adverb . If the method name contains more than one word,
then each word, except for the first one, should be capitalized
(Camel notation) .

13

Lesson 4. Loops

Examples:

void startprocess() {} // bad
void Start() {} // bad
void start_process() {} // bad
void sProcess() {} // bad
void sp() {} // bad
void start() {} // good
void startProcess() {} // good

14 3. Working with the Eclipse integrated debugger

3. Working with the Eclipse integrated debugger

Eclipse is a free integrated development environment
for  modular  cross-platform  applications .  It  is
developed and supported by the Eclipse Foundation .

Alternative Development Environments

Table 1. Rating of currently used Java-IDE

N

1

2

3

4

5

6

Name

% of votes

Evaluation

Eclipse

IntelliJ IDEA

NetBean

JBuilder

JDeveloper

JCreator

19.77

19.06

7.11

5.68

2.13

1.70

4.6

4.7

4.1

4.2

4.0

3.9

Table 2. Rating of previously used Java-IDE

N
1
2
3
4
5
6
7
8
9

Name

Eclipse
IntelliJ IDEA
NetBean
JBuilder
JDeveloper
Visual J++
JCreator
VisualAge for Java
JCreator

% of votes
21.47
16.64
14.22
11.66
7.11
5.26
4.26
3.69
3.41

Evaluation
3.0
3.3
2.9
3.5
2.8
1.8
2.3
2.8
2.0

15

Lesson 4. Loops

Eclipse Installation

From the http://www .eclipse .org/downloads/packages/

eclipse-ide-java-developers/indigor page .

Download Eclipse Indigo based on the type and size of

the operating system .

Download the archive . Unpack it in a convenient place:
Eclipse is not installed through the installer; it is portable,
unlike Netbeans .

Creating a Project in Eclipse

Run the Eclipse SDK and make sure that the Java projection

is open .

In  the  dialog  box  that  opens,  enter  the  name  of  the
project . The project will be located in the directory installed as
Workspace when configuring Eclipse . In the next dialog box,
go to the "Libraries" tab . We will not change anything here,
but remember that on this tab you can add additional Java
libraries to the project, and at the moment the standard API,
which is supplied with the JRE, is connected to the project .

Click "Finish" .

16 3. Working with the Eclipse integrated debugger

17

Lesson 4. Loops

An  empty  project  is  created,  in  order  to  continue  to
work, you need to add packages and classes . Create a package
through the context menu . Right click on the "src" folder and
select "New" -> "Package" . The same can be done by clicking
the "New Java Package" button on the toolbar . In the dialog
box that appears, enter the name of the package, it must be
unique to avoid collisions of names, as a rule, developers
invert the name of their domain, you too can do so . Create a
class through the context menu . Right click on the package
and select "New" -> "Class" . The same can be done by clicking
the "New Java Class" button on the toolbar . In the dialog box of
creating a class, enter its name, it can be anything, according to
the rules for naming classes in Java . Note the "public static void
main (String [] args)" option, thus we will tell the IDE to create
for us the same function . Click "Finish" . Now on the right,
we see the project structure and our class file with the JAVA
extension . The source code of the class is in the center, and the
class browser, showing the structure of packages and classes
in the form of a tree, is on the right . Edit the source code,
enter the instruction for outputting the line to the console —
System .out .println ("Your line"); . Do not forget to use hints
and autocompletion — start entering the code and press Ctrl
+ Space . Do not forget to end the line with ";" .

Save the changes by pressing the Ctrl + S keys .

Running the Java project in Eclipse

To check the efficiency of our program, click the "Run"
button on the toolbar or through the main menu . When you
first start, you need to select either to run the program as a
normal application, or as an applet . Select "Java Application" .

18 3. Working with the Eclipse integrated debugger

Your  first  Java  console  application  will  be  compiled  and
executed . In the "Console" view that opens, in the bottom
panel of the main window of the IDE, we will see the output
of the program, namely our line . Compiled class files with
the CLASS extension can be found in the folder with the ->
"bin" project .

19

Lesson 4
Loops

© Vitaliy Unguryan
© STEP IT Academy
     www.itstep.org

All rights to protected pictures, audio, and video belong to their authors or legal owners. Fragments of works are used exclusively in illustration purposes to the extent justified by the purpose as part of an educational process and for educational purposes in accordance with Article 1273 Sec. 4 of the Civil Code of the Russian Federation and Articles 21 and 23 of the Law of Ukraine “On Copyright and Related Rights”. The extent and method of cited works are in conformity with the standards, do not conflict with a normal exploitation of the work, and do not prejudice the legitimate interests of the authors and rightholders. Cited fragments of works can be replaced with alternative, non-protected analogs, and as such correspond the criteria of fair use.All rights reserved. Any reproduction, in whole or in part, is prohibited. Agreement of the use of works and their fragments is carried out with the authors and other right owners. Materials from this document can be used only with resource link.Liability for unauthorized copying and commercial use of materials is defined according to the current legislation of Ukraine.