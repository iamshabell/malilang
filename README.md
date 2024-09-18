## Malilang - first somali programming language

### What is Malilang?
Malilang is a programming language that is written in somali language. It is a simple language that is easy to learn and understand. It is designed to be used by somali people who are interested in programming but have difficulty understanding other programming languages.

### How does it work?
Malilang is a simple language that is based on the concept of functions and control flow. It has a simple syntax that is easy to understand and use. The language is designed to be easy to learn and use, so that somali people who are interested in programming can start writing code quickly.


## Basic Constructs
### Functions
- **hawl**: Defines a function

**Example:**
```javascript
hawl so_dhawoow(magac: xaraf) {
  daabac("Salaan " + magac)
}
```

### Control Flow
- **haddii**: If statement
- **haddii kale**: Else statement

**Example:**
```javascript
haddii (tiro > 0) {
    daabac("Tiradu waa togan")
} haddii kale {
    daabac("Tiradu ma aha togan")
}
```

### Loops(For)
- **dubbarid**: For loop

**Example:**
```javascript
dubbarid (i = 0; i < 10; i++) {
    daabac(i)
}
```

### Input/Output
- **daabac**: Print to the console
- **akhriso**: Read from the console

**Example:**
```javascript
daabac("Hello, World!")
```

### Booleans
- **haa**: True
- **may**: False

**Example:**
```javascript
weel xog_maku_jirtaa = haa
```

### Comments
- **//**: Single line comment
- **/* */**: Multi-line comment

### Example Program
```javascript
hawl soo-dhoweey(name: xaraf) {
    daabac("Soo-dhowey " + name)
}

dubbarid (i = 0; i < 5; i++) {
    haddii (i % 2 == 0) {
        daabac(i + " waa lambar togan")
    } haddii kale {
        daabac(i + " waa lambar aan toganayn")
    }
}

soodhoweey("Axmed")

weel togan_maku_jirto = haa
weel aan_toganayn = may

haddii (togan_maku_jirto) {
    daabac("Tiradu waa tiro togan")
} haddii kale {
    daabac("Tiradu ma aha tiro togan")
}
```
