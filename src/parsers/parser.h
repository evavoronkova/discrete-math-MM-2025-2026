#ifndef PARSER_H
#define PARSER_H

#include <string>
#include <filesystem>

#include "../graph.h"
using namespace std;

typedef boost::algorithm::detail::is_any_ofF<char> sep_type;

class parser {
public:
    virtual ~parser() = default;
    virtual graph parse(const string &file_path);
    bool is_has_metadata = false;
protected:
    bool is_first = true;
    const static boost::algorithm::detail::is_any_ofF<char> separator;
private:
    virtual void process(graph& graph, string& line) {}
};

class txt_parser : public parser {
    void process(graph& graph, string& line) override;
    const static inline sep_type separator = boost::is_any_of(" \t");
};

class csv_parser : public parser {
    void process(graph& graph, string& line) override;
    const static inline sep_type separator = boost::is_any_of(",");
};

class mtx_parser : public parser {
    void process(graph& graph, string& line) override;
    const static inline sep_type separator = boost::is_any_of(" ");
};

class uni_parser {
public:
    uni_parser() = delete;
    static graph parse(const string &file_path);
};
#endif // PARSER_H
