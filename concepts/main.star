// .star was used for this file, but .slang could be accepted as a file extension (because it's a slang for it lol)

import std::io; // usage of double ':' for package division

// the crab lang sure does it good enough
pub struct MyStruct {
		firstvar : int32 //however, i think the comma separation is kinda pointless, so we ditching that
		secondvar : int32 -> 34 //this initializes every instance of MyStruct with a secondvar field with the value 34
		thirdvar : string //let's hope i don't acciddentally make 13 string types

}

type thisisatypename -> MyStruct;

/*
		idealy the args string array is going to be optional and replaceable by a single string

		a variable name could be optionally denoted with a _ at the start, denoting that the developer does not want to use the value
*/

fn main(args : string[]) {
		const myvar : string = "this is a string and it's constant"; // hopefully anything that is constant during compile time, will be placed inside the binary of the executable file
		mut myothervar : thisisatypenamehaha -> thisisatypenamehaha{23, 12, "whatever"};

		static mynewvar : &string -> &myvar; //type is inferred from the refereced variable / constant

		/* the code from above is a piece that transfers the value from a variable into another by either using references using the & character after the variable that you're referencing, or by using the actual value using the * character */

		/*
				explaining the ideal behaviour from mut, static and const values
				mut variables are always mutable, no exceptions
				static variables are not mutable, however during declaration, they can accept values that aren't constant
				constant values cannot be changed and should receive values that either come from constant function or from type literals
		*/


		if myvar = *mynewvar {
				io::println("these lines are equal");
		}
}

pub fn second() : int32 {
		io::println("this comes from the second function");

		return 34;
}

//some more thoughts

/*
		anytime when declaring an integer, it should be possible to declare how many bits you want it to use, and that is by using 'int<number of bits>', with the number of bits being > 0 
		
		same thing for floats

*/
