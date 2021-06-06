// Good morning! Here's your coding interview problem for today.

// This problem was asked by Twitter.

// Implement an autocomplete system. That is, given a query string s and a set of all possible query strings, return all strings in the set that have s as a prefix.

// For example, given the query string de and the set of strings [dog, deer, deal], return [deer, deal].

// Hint: Try preprocessing the dictionary into a more efficient data structure to speed up queries.
package main

import "strings"

func wordSearch(searchText string, data []string) []string {
	dictionary := make(map[string][]string)
	for _, text := range data {
		firstChar := string(text[0])
		if val, ok := dictionary[firstChar]; ok {
			dictionary[firstChar] = append(val, text)
		} else {
			dictionary[firstChar] = []string{text}
		}
	}
	returnValue := []string{}
	if val, ok := dictionary[string(searchText[0])]; ok {
		for _, text := range val {
			if strings.HasPrefix(text, searchText) {
				returnValue = append(returnValue, text)
			}
		}
	}
	return returnValue
}
