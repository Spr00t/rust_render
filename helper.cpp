#include <QApplication>
#include <QWidget>
#include <functional>
#include <QImage>
#include <QPainter>
#include <QDebug>
#include <unistd.h>
#include <iostream>
#include "helper.h"
using namespace std;
class MyWidget : public QWidget
{
    int width;
    int height;
    uchar * data;
public:

    MyWidget(int width, int height, uchar * data):
        width(width),
        height(height),
        data(data)
    {
        auto rect = geometry();
        rect.setSize(QSize(width, height));
        setGeometry(rect);
    }
    ~MyWidget() {
        delete data;
    }

protected:
    virtual void	paintEvent(QPaintEvent * event)
    {
      QPainter painter(this);

      QImage image(width, height, QImage::Format_RGB32);
      for (int i = 0; i < height; i++) {
          memcpy(image.scanLine(i), &data[width * i * 4], width * 4);
      }
      painter.drawImage(QPoint(0,0), image);
    };


};

void showImage(void * img, int width, int height)
{
    uchar * data = new uchar [width * height * 4];
    memcpy(data, img, width * height * 4);
    auto * wid = new MyWidget(width, height, data);
    wid->show();

}
int exec(void * app)
{
    int i = 0;
    int res = static_cast<QApplication*>(app)->exec();
    delete static_cast<QApplication*>(app);
    return res;
}



void *application(int argc, char **argv)
{
    for (int i = 0; i < argc; i++) {
        printf("C++ argument %d %s\n", i, argv[i]);
    }
    int * argc_send = new int;
    *argc_send = argc;

    return static_cast<void*>(new QApplication(*argc_send, argv));
}
