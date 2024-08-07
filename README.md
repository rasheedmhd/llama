# Syntax

# Comments

```
// This is a comment
```

# Printing to Std Output

```
> print "Hello, world!";
Hello, world!
```

# DATA TYPES

# Booleans

```
> true; // Not false.
true
> false; // Not False
false
```

# Numbers

```
1234; // An integer.
12.34; // A decimal number
```

# Strings

```
"I am a string";
""; // The empty string.
"123"; // This is a string, not a number.
```

# No Value

```
nil
```

# Expressions

## Arithmetic

```
add + me;
subtract - me;
multiply * me;
divide / me;

-negateMe;
```

# Comparison and Equality

## Comparison Operators.

```
less < than;
lessThan <= orEqual;
greater > than;
greaterThan >= orEqual;

1 == 2; // false.
"cat" != "dog"; // true

314 == "pi"; // false

123 == "123"; // false
```

## Logical operators

```
!true; // false.
!false; // true.

true and false; // false.
true and true; // true.

false or false; // false.
true or false; // true.
```

# Precedence and Grouping

```
var average = (min + max) / 2;
```

# Statements

```
print "Hello, world!";

{
    print "One statement.";
    print "Two statements.";
}

```

# Variables
Llama is dynamically typed, so you can assign a value of any type to variables declared prior.
```

var breakfast 	= "bagels";
print breakfast; // "bagels".

breakfast 	= "beignets";
print breakfast; // "beignets".

```

# Shadowing
```

var breakfast = "bagels";
> print breakfast;
bagels;
var breakfast = "croissant";
> print breakfast;
croissant

```


# Control Flow

## if

```
if (condition) {
    print "yes";
} else {
    print "no";
}
```

## while

```
var a = 1;

while (a < 10) {
    print a;
    a = a + 1;
}
```

## for

```
for (var a = 1; a < 10; a = a + 1) {
    print a;
}
```

# Functions

```
// Functions that take arguments
makeBreakfast(bacon, eggs, toast);

// Functions that do not toake arguments
makeBreakfast();
```

# fun keyword

```
fun printSum(a, b) {
    print a + b;
}

fun returnSum(a, b) {
    return a + b;
}
```

# Closures

```

fun addPair(a, b) {
    return a + b;
}
fun identity(a) {
    return a;
}

print identity(addPair)(1, 2); // Prints "3"

fun outerFunction() {
    fun localFunction() {
        print "I'm local!";
    }
    localFunction();
}

fun returnFunction() {
var outside = "outside";
    
    fun inner() {
        print outside;
    }
    
    return inner;
}

var fn = returnFunction();
fn();

```

# Classes

```
// The body of a class contains its methods.
// They look like function declarations but without the fun keyword.
class Breakfast {
    
    cook() {
        print "Eggs frying!";
    }
    
    serve(who) {
        print "Enjoy your breakfast, " + who + ".";
    }
    
}
```

## Classes are First Class

```

// Storing Classes in Variables.
var someVariable = Breakfast;

// Passing Classes to Functions.
someFunction(Breakfast);

```

## Class Instances

```
// Calling a class like a function, produces a new instance of itself.
var breakfast = Breakfast();
print breakfast; // "Breakfast instance".
```

## Instantiation and Initialization

```

// Assigning to a field creates it if it doesn’t already exist
breakfast.meat = "sausage";
breakfast.bread = "sourdough";

// If you want to access a field or method on the current object
// from within a method, you use good old this.

class Breakfast {
    serve(who) {
        print "Enjoy your " + this.meat + " and " +
        this.bread + ", " + who + ".";
    }
    // ...
}

class Breakfast {
    init(meat, bread) {
        this.meat = meat;
        this.bread = bread;
    }
    // ...
}

var baconAndToast = Breakfast("bacon", "toast");

baconAndToast.serve("Dear Reader");
// "Enjoy your bacon and toast, Dear Reader."

```

# Inheritance

```

class Brunch < Breakfast {
    drink() {
        print "How about a Bloody Mary?";
    }
}

// Every method defined in the superclass is also available to its subclasses.
var benedict = Brunch("ham", "English muffin");
benedict.serve("Noble Reader");

// Even the init() method gets inherited.
// In practice, the subclass usually wants to define its own init() method too.
class Brunch < Breakfast {
    init(meat, bread, drink) {
        super.init(meat, bread);
        this.drink = drink;
    }
}

```