#include<stdio.h>
#include<stdint.h>
#include<stdlib.h>


int edgeScore(int* edges, int edgesSize) {
    uint32_t *edgeScoreArray = (uint32_t *)calloc(edgesSize, sizeof(uint32_t));
    int index_of_max_score = 0;
    uint32_t max_score = 0;
    for (size_t i = 0; i < edgesSize; i++){
        edgeScoreArray[edges[i]] += i;
        if(edgeScoreArray[edges[i]] > max_score){
            index_of_max_score = edges[i];
            max_score = edgeScoreArray[edges[i]];
        }else if(edgeScoreArray[edges[i]] == max_score && edges[i] < index_of_max_score){
            index_of_max_score = edges[i];
        }
    }

    free(edgeScoreArray);
    
    return index_of_max_score;
}
