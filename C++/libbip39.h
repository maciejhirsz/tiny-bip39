#ifdef __cplusplus
extern "C" {
#endif
  extern char bip39_generate_new_seed            (unsigned char c_mneumonic_words, unsigned char *pca_phrase, unsigned char *pc_phrase_size,   unsigned char *pca_seed);
  extern char bip39_regenerate_seed_from_mnemonic(unsigned char *pca_phrase, unsigned char *pc_phrase_length, unsigned char *pca_seed);
#ifdef __cplusplus
}
#endif