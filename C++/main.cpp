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
  printf("}\n");  
  
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

// Return codes: 0 - success
//              -1 - Input error
//              -2 - function call failed
int evaluate_mneumonic(char *pca_phrase, uint8_t cLength)
{
  int8_t cReturnCode; 
  uint8_t ca_seed[64];
  
  memset(&ca_seed,0,64);  
  
  if (pca_phrase[cLength] != 0)
  {
    return -1;
  }

  cReturnCode = bip39_regenerate_seed_from_mnemonic( (uint8_t *)pca_phrase, &cLength, &ca_seed[0]);
  switch (cReturnCode)
  {
    case 0: // Success
      break;
    case -1:
      printf("bip39_regenerate_seed_from_mnemonic() string length invalid\n");
      return -1;
    case -2:
      printf("bip39_regenerate_seed_from_mnemonic() Invalid input: word count must be 12,18 or 24\n");
      return -2;
    case -3:
      printf("bip39_regenerate_seed_from_mnemonic() Internal error: Incorrect phrase length\n");
      return -2;
    case -4:
      printf("bip39_regenerate_seed_from_mnemonic() Internal error: Incorrect seed length\n");
      return -2;
    case -5:
      printf("bip39_regenerate_seed_from_mnemonic() Could not regenerate the seed from the input phrase\n");      
      return -2;
    default:
      printf("bip39_regenerate_seed_from_mnemonic() Unknown return code: %d\n",cReturnCode);
      return -2;
  }

  printf("Regenerated seed from the mnemonic:\n     {");
  for (int iI=0;iI<64;iI++)
  {
    printf("0x%02x",ca_seed[iI]);
    if (iI<63)
      printf(",");
  }
  printf("}\n\n");  
  
  return 0;
}

int main()
{
  char caData[255];
  int8_t cReturnCode;
  
  printf("---- 24 word mneumonic ----\n");
  perform_test(24);
  
  printf("\n\n---- 18 word mneumonic ----\n");
  perform_test(18);
  
  printf("\n\n---- 12 word mneumonic ----\n");
  perform_test(12);

  printf("\n\n\n---- evaluate a mneumonic : Expect checksum failure ----\n");  
  //Swopped the first 2 words to trigger invalid checksum
  sprintf(&caData[0],"brisk detail range elder useful nose claw venue erase neglect settle funny maze tired claw fortune comfort tip deny flight joke physical avocado explain");
  cReturnCode = evaluate_mneumonic(&caData[0], strlen(caData));
  if (cReturnCode == -2)
  {
    printf("Checksum error detected correctly\n");
  }
  else
  {
    printf("Expected a function failure. Received %d instead\n",cReturnCode);
  }
  
  printf("\n\n---- evaluate a mneumonic : Expect success ----\n");  
  //Swopped the first 2 words to trigger invalid checksum
  sprintf(&caData[0],"detail brisk range elder useful nose claw venue erase neglect settle funny maze tired claw fortune comfort tip deny flight joke physical avocado explain");
  cReturnCode = evaluate_mneumonic(&caData[0], strlen(caData));
  if (cReturnCode == 0)
  {
    printf("Success\n");
  }
  else
  {
    printf("Expected success. Received %d instead\n",cReturnCode);
  }  
  
  return 0;
}