// wx raw: a chat-like window drawn on one QRasterWindow, so accessibility sees no field and no focused element, as in WeChat: a search box at the top and a chat input at the bottom that holds the focus at start. With "reset" as the third argument, the window gives the chat input the focus whenever it gains focus. Focus, activation and text append to ~/wxfocus/raw.log.
#include <QDateTime>
#include <QDir>
#include <QFile>
#include <QGuiApplication>
#include <QKeyEvent>
#include <QMouseEvent>
#include <QPainter>
#include <QRasterWindow>
#include <QTextStream>

static void logLine(const QString &s) {
    QDir().mkpath(QDir::homePath() + "/wxfocus");
    QFile f(QDir::homePath() + "/wxfocus/raw.log");
    if (f.open(QIODevice::Append | QIODevice::Text)) QTextStream(&f) << QDateTime::currentMSecsSinceEpoch() << " " << s << "\n";
}

class Chat : public QRasterWindow {
public:
    explicit Chat(bool reset) : reset(reset) {}

protected:
    void paintEvent(QPaintEvent *) override {
        QPainter p(this);
        p.fillRect(QRect(0, 0, width(), height()), QColor(236, 236, 236));
        draw(p, searchBox(), search, focus == 0, "Search");
        draw(p, inputBox(), input, focus == 1, "");
    }
    void mousePressEvent(QMouseEvent *e) override {
        QPoint at = e->position().toPoint();
        logLine(QString("press %1,%2").arg(at.x()).arg(at.y()));
        if (searchBox().contains(at)) setField(0, "click");
        else if (inputBox().contains(at)) setField(1, "click");
    }
    void keyPressEvent(QKeyEvent *e) override {
        if (e->text().isEmpty()) return;
        QString &field = focus == 0 ? search : input;
        field += e->text();
        logLine(QString(focus == 0 ? "search=" : "input=") + field);
        update();
    }
    void focusInEvent(QFocusEvent *e) override {
        logLine(QString("focusIn reason %1").arg(e->reason()));
        if (reset) setField(1, "reset on focusIn");
    }
    void focusOutEvent(QFocusEvent *e) override { logLine(QString("focusOut reason %1").arg(e->reason())); }

private:
    QRect searchBox() const { return QRect(20, 20, width() - 40, 30); }
    QRect inputBox() const { return QRect(20, height() - 200, width() - 40, 180); }
    void setField(int f, const char *why) {
        if (focus != f) logLine(QString("field %1 -> %2 (%3)").arg(name(focus), name(f), why));
        focus = f;
        update();
    }
    static QString name(int f) { return f == 0 ? "search" : "input"; }
    static void draw(QPainter &p, QRect r, const QString &text, bool focused, const QString &hint) {
        p.fillRect(r, Qt::white);
        p.setPen(focused ? QPen(QColor(7, 193, 96), 2) : QPen(Qt::gray, 1));
        p.drawRect(r);
        p.setPen(text.isEmpty() ? Qt::gray : Qt::black);
        p.drawText(r.adjusted(6, 0, -6, 0), Qt::AlignLeft | Qt::AlignVCenter, text.isEmpty() ? hint : text);
    }
    bool reset;
    int focus = 1;
    QString search, input;
};

int main(int argc, char **argv) {
    QGuiApplication app(argc, argv);
    QObject::connect(&app, &QGuiApplication::applicationStateChanged, [](Qt::ApplicationState s) { logLine(QString("state %1").arg(int(s))); });
    Chat chat(argc > 3 && QString(argv[3]) == "reset");
    chat.setTitle("wx raw");
    chat.resize(420, 560);
    chat.setPosition(argc > 2 ? atoi(argv[1]) : 400, argc > 2 ? atoi(argv[2]) : 168);
    chat.show();
    return app.exec();
}
