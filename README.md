# slowc
The Slow programming language. \
This is the Slow compiler (slowc)

## Why "Slow"?
idk, i just want to make a compiler.

## Usage
The compiler can be used in both Hebrew and English, depending on what you compile it for
For hebrew compile with: `cargo build -r --features hebrew` \
for english compile with: `cargo build -r --features english`

## Some documentation

### Functions
```
func <ATTRIBUTES> <IDENTIFIER> <RETURN_TYPE> (<ARGUMENTS>)
{
	<CODE>
}
```
for example:
```
func global main() -> i32
{
}
```
or in hebrew:
```
פונקציה גלובלי ראשי() -> חתום32
{

}
```