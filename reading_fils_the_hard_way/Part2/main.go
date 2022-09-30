package main

import (
	"io/ioutil"
	"fmt"
)

func main(){
	payload, err := ioutil.ReadFile("/etc/hosts")
	if err != nil{
		panic(err)
	}

	fmt.Printf("%s\n", string(payload))
}
