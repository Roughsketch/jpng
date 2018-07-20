# jpng
Library for handling JPNG files.

Currently this crate only supports splitting and saving individual parts of the JPNG.

# Usage
To create a JPNG, you pass in a path to the constructor.

```rust
let jpng = jpng::Jpng::new("example.jpng")?;
```

This method will fail if the file can't be found, or if the given file is a malformed JPNG. From there, you can save either the image (JPEG), mask (PNG), or the combined image by using the following:

```rust
jpng.save("output"); // Save the combined image as the file "output.png"
jpng.save_image("image"); // Save the image portion of the JPNG as "image.jpg"
jpng.save_mask("mask"); // Save the mask portion of the JPNG as "mask.png"
```
