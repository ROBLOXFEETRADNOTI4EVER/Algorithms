
Hash Table = collection of key value Pairs
	Key Value Pair = Entry<K,V>
	


![[Pasted image 20251109203816.png]]
For this exmaple the Key wil be an integer and the Value is a String

Key.hash


![[Pasted image 20251109204209.png]]

![[Pasted image 20251109204438.png]]



# Hashtable & Hashing - Colorful Obsidian Cheat Sheet

## 🔹 Hashtable

A **data structure** that stores **unique keys** mapped to values.  
Example: `<i32, &str>`

- Each key/value pair is called an ==Entry==
- **Fast** operations: ==insertion, lookup, deletion==
- **Not ideal** for very small datasets, **great** for large datasets

**Runtime Complexity:**

- **Best case:** `O(1)`
- **Worst case:** `O(n)`

---

## 🔹 Hashing

**Definition:** Convert a key into an **integer** (hash code) based on its value and type.

- In a **Hashtable**, we often use:

```java
index = key.hashCode() % capacity;
```

- This index tells us **where to store** the entry.

---

## 🔹 Bucket

- A **bucket** is a storage location at a given index.
- Can store **multiple entries** if collisions happen
- Often implemented as a **linked list**

---

## 🔹 Collision

Occurs when **two keys hash to the same index**.
- More collisions → slower performance
- Less collisions → more efficiency

**Handling Collisions:**

- **Chaining:** store multiple entries in the same bucket (linked list)
- **Open addressing:** find another free spot (linear probing, etc.)
---

## 🔹 Extra Notes

- Hash functions should be ==fast and uniform==
- Capacity often chosen as a ==prime number== to reduce collisions
- Good for ==dictionaries, caches, and sets==