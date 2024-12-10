#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>

int main(int argc, char **argv) {
    int cookTimeSeconds = atoi(argv[1]);
    int cookTimeMinutes = cookTimeSeconds / 60;
    int cookTimeSecondsDisplay = cookTimeSeconds % 60;
    int frequencyHertz = 10;
    int waitTimeUs = 1000000 / frequencyHertz;
    int repetitionCount = cookTimeSeconds * frequencyHertz;
    printf("COOK TIME: %02d:%02d\n", cookTimeMinutes, cookTimeSecondsDisplay);
    printf("BEEP!\n");
    for (int i = 0; i < repetitionCount; i++) {
        printf("m");
        fflush(stdout);
        usleep(waitTimeUs);
    }
    printf("\nBEEEP! BEEEP! BEEEP!\nFood's done! :)\n");
    return 0;
}
