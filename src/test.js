// // SCOPING
var a = "global a";
var b = "global b";
var c = "global c";

{
    var a = "outer a";
    var b = "outer b";

    {
        var a = "inner a";
        var b = "inner b";
        print a;
        print b;
        print c;
    }

    print a;
    print b;
    print c;
}

print a;
print b;
print c;
print dropped;



// var test = 1;
// {
//     print test;
//     test = "test + 2 inside {}";
//     print test;
// }
// print test;
// var a = 0;
// // var temp = 0;
// //
// // for (var b = 1; a < 10000; b = temp + b) {
// print a;

// a = "ass";
//     print a;
//     temp = a;
//     a = b;
// }

// var x = 7; print x; { print x; bush = "name"; print bush; } print x;

// for (var i = 1; i <= 5; i + 1) { print i; }

// Llama while can do for loops, like so
{
    var i = 0;
    while (i < 10)
    {
        print i;
        i = i + 1;
        print i;
    }
}

// var i = 0; for (; i < 10; i = i + 1) print i;