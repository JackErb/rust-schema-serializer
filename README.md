Rust serializer / deserializer built using procedural macros.

Defines the procedural macro #[derive(Schematize)] which can be added to structs/enums.

    Functions includes:
      serialize
       - generates a schematized object representation of the object
      deserialize
       - deserializes the schematized object into an instance of the item
      schema_default
       - generates a default version of the struct, respecting any #[schema_default(...)] markup

Serializes Rust structures into a JSON-like structure than can be read and deserialized back into their runtime form.

Supports dynamically-sized arrays and strings which are allocated onto the heap.
