//
// Created by anton on 05.10.2021.
//

#include "stdio.h"
#include "../target/merk.h"
#include "./hexutils.c"
#include "assert.h"

int main (void) {
    char *proof_hex = "01761149f5816723fdc7025790d285f63bbe26acb3471e57f28fa4db6e4859c3ae02887fcd3a7fef9b356dd12fc2e4d58c54c9e42908070dee2aaf7c7b5d389f736010017d1db154d2a87f5f5136a1b8581759b75e72d6047ee3efbfcba0889c2d4e8b6302b1a68c50747a42fd140dedcabb7c4ff3ebed1c729a1541ba1b44f1ad7c24a3e21003206c05f39cee3a2c1436b61a0746503a743658b2d0e76b432e741f9bbbe211dc34008001000000a462696458206c05f39cee3a2c1436b61a0746503a743658b2d0e76b432e741f9bbbe211dc346762616c616e63651a3b9ac7f4687265766973696f6e006a7075626c69634b65797381a36269640064646174615821032d6d975393f17c0d605efe8562c06cbfc913afcc73d0d855399c0a97d776154064747970650002163fc42a48f26886519b6e64280729c5246d92dad847823faef877eb282c7fb81001d715565e9f71ae94fe2d07568d1e2fd1043bca07c2da385dcb430cb84f92882211022325a14555b8403767a314c3bc9b8708a25e2bc756cecadf56e5184de8dcc3a31001b51e23fbb805bfd917bb0e131da4488c48417dbe82bd6d7e9d69a50abd77a3c31102f41f6cae67288cccacc79ab5c2c29fd6ec3b83919625131ec9139c65606849c61001f234e77a4845b865816729fa14801189395d2ce658c1a24130f45b076d8f047a11111102c97ff70a287f4d9741f5c54e5fc5e6a365043cdbedf623ae7d0e280a6a32b70b10018a28f5bebdbf987079878315cde74e22ef591983a576d3c6e2807ae1fd12ff8811";

    unsigned char *proof_bin = hex2bin(proof_hex);

    printf("Serialized: \n");
    char *serialized_proof = bin2hex(proof_bin, 1144 / 2);
    printf("%s", serialized_proof);
    printf("\n");

    printf("Original: \n");
    printf("%s", proof_hex);
    printf("\n");

    //assert(serialized_proof == proof_hex);

    ExecuteProofResult *result = execute_proof_c(proof_bin, 1144 / 2);

    printf("element_count: \n");
    printf("%lu", result->element_count);
    printf("\n");

    Element *first_element = result->elements[0];
    char *element_hex = bin2hex(first_element->value, 128);

    char *expected_value_hex = "01000000a462696458206c05f39cee3a2c1436b61a0746503a743658b2d0e76b432e741f9bbbe211dc346762616c616e63651a3b9ac7f4687265766973696f6e006a7075626c69634b65797381a36269640064646174615821032d6d975393f17c0d605efe8562c06cbfc913afcc73d0d855399c0a97d7761540647479706500";

    printf("element_value: \n");
    printf("%s", element_hex);
    printf("\n");

    printf("expected element_value: \n");
    printf("%s", expected_value_hex);
    printf("\n");
}