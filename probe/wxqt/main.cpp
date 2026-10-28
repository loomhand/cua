// wx qt: a chat-like Qt Widgets window: a search line edit at the top-left, a list, and a chat input that holds the focus at start; focus moves, presses and text changes append to ~/wxfocus/qt.log.
#include <QApplication>
#include <QDateTime>
#include <QDir>
#include <QFile>
#include <QLineEdit>
#include <QListWidget>
#include <QPlainTextEdit>
#include <QTextStream>
#include <QVBoxLayout>
#include <QWidget>

static void logLine(const QString &s) {
    QDir().mkpath(QDir::homePath() + "/wxfocus");
    QFile f(QDir::homePath() + "/wxfocus/qt.log");
    if (f.open(QIODevice::Append | QIODevice::Text)) QTextStream(&f) << QDateTime::currentMSecsSinceEpoch() << " " << s << "\n";
}

static QString nameOf(QObject *o) { return o ? o->objectName() : QString("none"); }

int main(int argc, char **argv) {
    QApplication app(argc, argv);
    QWidget win;
    win.setWindowTitle("wx qt");
    auto *search = new QLineEdit;
    search->setObjectName("search");
    search->setPlaceholderText("Search");
    search->setAccessibleName("Search");
    auto *list = new QListWidget;
    list->setObjectName("list");
    for (int i = 0; i < 6; i++) list->addItem(QString("chat %1").arg(i));
    auto *input = new QPlainTextEdit;
    input->setObjectName("input");
    input->setAccessibleName("Message");
    auto *box = new QVBoxLayout(&win);
    box->addWidget(search);
    box->addWidget(list);
    box->addWidget(input);
    QObject::connect(&app, &QApplication::focusChanged, [](QWidget *from, QWidget *to) { logLine("focus " + nameOf(from) + " -> " + nameOf(to)); });
    QObject::connect(&app, &QApplication::applicationStateChanged, [](Qt::ApplicationState s) { logLine(QString("state %1").arg(int(s))); });
    QObject::connect(search, &QLineEdit::textChanged, [](const QString &t) { logLine("search=" + t); });
    QObject::connect(input, &QPlainTextEdit::textChanged, [input] { logLine("input=" + input->toPlainText()); });
    win.resize(420, 560);
    win.move(argc > 2 ? atoi(argv[1]) : 400, argc > 2 ? atoi(argv[2]) : 168);
    win.show();
    input->setFocus();
    return app.exec();
}
