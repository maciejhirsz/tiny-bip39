#include <stdio.h>
#include <unistd.h>
#include <string> 
#include <string.h>
#include <cstdlib>
#include <cstdint>
#include "main.h"

#include "libbip39.h"

// Return code: 0 - success
//             -1 - Input error
//             -2 - Mneumonic could not be regenerated from the seed
int perform_test(uint8_t cMneumonicSize)
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

  if ((cMneumonicSize != 24) && (cMneumonicSize != 18) && (cMneumonicSize != 12))
  {
    printf("Invalid input. Mneumonic must be 12,18 or 24 words\n");
    return -1;
  }
  
  printf("Generate a new random seed and mnemonic of the seed:\n");
  char cReturnCode = bip39_generate_new_seed(cMneumonicSize, &ca_phrase[0], &cSize, &ca_seed[0]);
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
    printf("Error generating seed from mnemonic. ReturnCode=%d\n",cReturnCode);
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
    return 0;
  }
  else
  {
    printf("The original and regenerated seeds doesn't match\n");
    return -2;
  }
}

int evaluate_mneumonic()
{
  int8_t cReturnCode; 
  uint8_t cSize;
  unsigned char ca_phrase[254];
  unsigned char ca_seed2[64];
  
  memset(&ca_phrase[0],0x55,254);
  memset(&ca_seed2,0,64);  
  
  sprintf( (char *)&ca_phrase[0], "brisk detail range elder useful nose claw venue erase neglect settle funny maze tired claw fortune comfort tip deny flight joke physical avocado explain");
  
  cSize = strlen( (char *)&ca_phrase[0] );
  try
  {
    cReturnCode = bip39_regenerate_seed_from_mnemonic(&ca_phrase[0], &cSize, &ca_seed2[0]);
  }
  catch(...)
  {
    cReturnCode=-1;
  }
  if (cReturnCode != 0)
  {
    printf("Error generating seed from mnemonic. ReturnCode=%d\n",cReturnCode);
    exit(0);
  }
  printf("Regenerated seed from the mnemonic:\n     {");
  for (int iI=0;iI<64;iI++)
  {
    printf("0x%02x",ca_seed2[iI]);
    if (iI<63)
      printf(",");
  }
  printf("}\n\n");  
  
  return 0;
}

int main()
{
  printf("---- 24 word mneumonic ----\n");
  perform_test(24);
  
  printf("\n\n---- 18 word mneumonic ----\n");
  perform_test(18);
  
  printf("\n\n---- 12 word mneumonic ----\n");
  perform_test(12);

  printf("\n\n---- evaluate a mneumonic ----\n");
  evaluate_mneumonic();
  
  return 0;
}