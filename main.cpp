#include <helper.h>
#include <string.h>
int main(int argc, char ** argv) {
    void * app = application(argc, argv);
    int w = 200;
    int h = 300;
    void * img = new char[w * h * 4];
    for (int i = 0; i < w * h * 4; i++) {
        ((int *)img)[i / 4] = 0xff00ff00;
    }
    showImage(img, w, h);
    return exec(app);

}
