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

    char *queryKey0 = "b2c4d80c2d16da67cb42ad5765513668b4fc524d";
    char *queryKey1 = "9d1560ea9f6e6d33ac4ebf6e2acbb8cdb5f54a5d";
    char *queryKey2 = "d3afa1ca0536ebbf69ba031272e6a80413bdb717";
    char *queryKey3 = "6e0f6ad70fc0d424e0a91dc67652ddaf9ca41dd7";
    char *queryKey4 = "d8fc5dd8e33d461954b8a2552e600ab93e70be6f";

    unsigned char *queryKeyBin0 = hex2bin(queryKey0);
    unsigned char *queryKeyBin1 = hex2bin(queryKey1);
    unsigned char *queryKeyBin2 = hex2bin(queryKey2);
    unsigned char *queryKeyBin3 = hex2bin(queryKey3);
    unsigned char *queryKeyBin4 = hex2bin(queryKey4);

    Query *query0 = malloc(sizeof(Query));
    Query *query1 = malloc(sizeof(Query));
    Query *query2 = malloc(sizeof(Query));
    Query *query3 = malloc(sizeof(Query));
    Query *query4 = malloc(sizeof(Query));

    query0->key_length = 20;
    query1->key_length = 20;
    query2->key_length = 20;
    query3->key_length = 20;
    query4->key_length = 20;

    query0->key = queryKeyBin0;
    query1->key = queryKeyBin1;
    query2->key = queryKeyBin2;
    query3->key = queryKeyBin3;
    query4->key = queryKeyBin4;

    Keys *keys = malloc(sizeof(Keys));
    keys->element_count = 5;
    keys->elements = malloc(sizeof(Query*)*5);
    keys->elements[0] = query0;
    keys->elements[1] = query1;
    keys->elements[2] = query2;
    keys->elements[3] = query3;
    keys->elements[4] = query4;

    char *proof_hex_5_160bit_hashes = "018604c121970b1dd7ff9682c9584dcb328eb5b6a4ceb3f5c50dc4a30881ec029902fbf32b582538666ca797807cd087df7e16caeffebfc40a94448f62e370ef7bba1003146bcfdf839d58e08ff0128c19d81e450a19b09a2100201c6f400f342d9936f39ad8082344e0aede119f45bdfa4d21811bec243c04200903146f339bc9d3fccc45c09a9678de642d620ba8624c0020575a3be407c7b43aea23a34891a5dd0e4d21f97d1a5a113caf475e0ee5bd80a31001dda3a52ce526f6f3f195ef899d4c20c9c2763e35eeb6ef2047a2f44f6ff9e74f110274d5cda9db960da8eebe9de190de382707eb35196776f133d76a73731ef34702100185f350bf3690cf21c51820811533e3949ba238b2aa55ed57b650eb57211185941102c1a461d30f21383643451cb8a99a03f946e23884799a7a68c172f7c7128ca2071003149c7868515d684da9468d6eaff354902795c1509800202947ab16436bb350747029bb2b317131d564ef524c33f4ad427b406a543c084e03149e3e4dfa8682d9d5f94516ed9d5ca008e18754910020f4e3b50bd0f73395689ceb3d1fc930987580850becb08945885a57311cbb05051001885e7e3dee35bbf4735b252767aafb4ed81332f34c24556cb058d64ce933ed541102fd0ca4e90c8c7e80095b9a02c3fbae2a017f8a27efdb572d3e08144693693e37100314b1386c941706cb5cf5446bd0ba449a4d44beab1600204daae7f0fb8e0b8df9eb54fe33cfe44a11cebdbe7987efebbf9be93b0320d5d50314b32efd4249df7f2197d08035ae1df2bc58d8c1cc0020878122e15567c6d2b3a1bebf86c7fc6b0c401dc80444406e10678f43e91b546b1001eff524fbd1f3b03df81fa6a1c9ebd19f50a99c28fda7ed830e0f53352fe7881c111102e9010df939d28f3fd03cc1c42d1da8095069c9243dc85015cf13648d78dd31581001a620de290694dadc8db87fa26f131b2f51b863e2a45624166305443d94a38c2c0314cfc46d8fcf142fb4a25700983e912bd3bac8d7eb00207376e2b96c26eff8be40be134030b7d31769ef7dd038da93850a9636fac6f1a6100314d3d8aebf222e2350db1a6b119134f24efe0627ce002096b06816790899354d5de7ba3a4bf1f482ff8594ffc35f81aff4b6b9c8fa09ed110314dfd0a3cf05a7cd1547bab95b1d717c05387253190020a561db76723b888a9412d7bae6c721319b0ab04904da4328611f28dad438a42910015d65ec7a5efe14a87fc79b3d74795fab716410a3aaa9278ee3947f2d648da3d51102a725e92fec23c188b4931a3c45430e0917ef2de7cd2fb9306458f5d4d48f0e2f100121af2df2423288f917ea6d4cde07587edead17292bf45e747274946b7007fa8111111111";

    unsigned char *proof_bin_5_160bit_hashes = hex2bin(proof_hex_5_160bit_hashes);

    ExecuteProofResult *result3 = execute_proof_query_keys_c(proof_bin_5_160bit_hashes, 1976 / 2, keys);

    printf("--------------\n5 hashes\n--------------\n");
    printf("valid: \n");
    printf("%d\n", result3->valid);

    printf("element_count: \n");
    printf("%lu", result3->element_count);
    printf("\n");
}