Build
  Library
  =======
  First build the bip39 rust crate, by executing 'cargo build'
  in the top directory. To build a release (non debug), use
  the command: cargo build --release
  The debug build is inside target/debug and 
  the release build is inside target/release

  C++ example
  ===========
  The build environment assumes Gnu/Linux with the C++ compiler
  preinstalled. On Debian & derivatives, run 'apt-get install
  build-essential' to install the basic compiler and build
  environment.

  The header file for libbip39 is included in this directory.
  Its called libbip39.h. To use the library with other projects
  you need to include this H file.

  Run 'make'
  ./generate_seed_debug   -- execute the 'debug' build
  ./generate_seed_release -- executes the 'release' build

Using the library
=================
Generate a new mnemonic
=======================
char bip39_generate_new_seed (unsigned char *pca_phrase, unsigned char *pc_phrase_size,   unsigned char *pca_seed);
  The function generates a new mnemonic & seed. The results are placed into
  two character arrays that are passed to the function. 
  pca_phrase - Array of up to 256 bytes in which the function can store the
               mnemonic. The mnemonic string is null-terminated.
  pc_phrase_size - Size of the array that you provide to the function. The function 
                   checks if the generated mnemonic will fit into the array before
                   copying the data into the array. The function will fail if the 
                   array is not large enough
  pca_seed - Array of 64 bytes in which the function can store the seed. The
             array is treated as a raw data array that is not
             null-terminated.
  return code - The function has the following return codes:
                0 - Success
               -1 - Library failure
               -2 - Not enough space in pca_phrase to store the mnemonic

char bip39_regenerate_seed_from_mnemonic(unsigned char *pca_phrase, unsigned char *pc_phrase_length, unsigned char *pca_seed);
  The function takes the mnemonic as input and regenerate the seed.
  The result is placed into the pca_seed array passed to the function
  pca_phrase - A null-terminated string containing the mnemonic.
  pc_phrase_length - Length of the string that is supplied to the function.
  pca_seed - Array of 64 bytes in which the function can store the seed. The
             array is treated as a raw data array that is not
             null-terminated.
  return code - The function has the following return codes:
                0 - Success
               -1 - Could not parse the mnemonic phrase
               -2 - Library failure

Linker
=====
Linking multiple rust libraries in a C or C++ program causes this 
error:  lib.rs:105: multiple definition of `rust_eh_personality';

Two work arounds are proposed: When using debug builds of the
rust libraries, they will link in the C/C++ app without giving
this error.

Alternatively, use these linker flags:
  LDFLAGS="-Wl,--allow-multiple-definition"

This was discussed at: https://github.com/rust-lang/rust/issues/44322