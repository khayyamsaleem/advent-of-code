#include "get_input.h"
#include <curl/curl.h>
#include <sstream>
#include <iostream>

size_t write_callback(void* contents, size_t size, size_t nmemb, std::string* s) {
    size_t new_length = size * nmemb;
    try {
        s->append((char*)contents, new_length);
        return new_length;
    } catch (std::bad_alloc& e) {
        return 0;
    }
}

std::string get_input(const std::string& session_token, int year, int day) {
    std::string url = "https://adventofcode.com/" + std::to_string(year) + "/day/" + std::to_string(day) + "/input";
    std::string response;

    CURL* curl = curl_easy_init();
    if (curl) {
        curl_easy_setopt(curl, CURLOPT_URL, url.c_str());
        curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_callback);
        curl_easy_setopt(curl, CURLOPT_WRITEDATA, &response);
        curl_easy_setopt(curl, CURLOPT_COOKIE, ("session=" + session_token).c_str());

        CURLcode res = curl_easy_perform(curl);
        if (res != CURLE_OK) {
            std::cerr << "curl_easy_perform() failed: " << curl_easy_strerror(res) << std::endl;
        }

        curl_easy_cleanup(curl);
    }
    return response;
}
