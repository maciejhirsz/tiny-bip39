#include <stdio.h>
#include <unistd.h>
#include <string> 
#include <string.h>
#include <cstdlib>
#include <cstdint>
#include "main.h"

#include "libbip39.h"

int main()
{
  int16_t iReturnCode; 
  uint8_t cSize;
    
    
  unsigned char ca_phrase[254];
  memset(&ca_phrase[0],0x55,254);
  cSize = sizeof(ca_phrase);
  
  unsigned char ca_seed[64];
  memset(&ca_seed,0,64);
  unsigned char ca_seed2[64];
  memset(&ca_seed2,0,64);  
  
  printf("Generate a new random seed and mnemonic of the seed:\n");
  char cReturnCode = bip39_generate_new_seed(&ca_phrase[0], &cSize, &ca_seed[0]);
  if(cReturnCode != 0)
  {
    printf("Error generating new seed\n");
    exit(0);
  }
  
  printf("New mnemonic:\n     %s\n",&ca_phrase[0]); 
  printf("New seed:\n     {");   
  for (int iI=0;iI<64;iI++) {
    printf("0x%02x",ca_seed[iI]);
    if (iI<63)
      printf(",");
  }
  printf("}\n\n");  
  
  cSize = strlen( (char *)&ca_phrase[0] );
  cReturnCode = bip39_regenerate_seed_from_mnemonic(&ca_phrase[0], &cSize, &ca_seed2[0]);
  if (cReturnCode != 0)
  {
    printf("Error generating seed from mnemonic\n");
    exit(0);
  }
  printf("Regenerate the seed from the mnemonic:\n     {");
  for (int iI=0;iI<64;iI++)
  {
    printf("0x%02x",ca_seed2[iI]);
    if (iI<63)
      printf(",");
  }
  printf("}\n\n");  
  
  if (strncmp ( (char *)&ca_seed[0], (char *)&ca_seed2[0], 64) == 0)
  {
    printf("The original and regenerated seeds match\n");
  }
  else
  {
    printf("The original and regenerated seeds doesn't match\n");
  }
  
  return 0;
}