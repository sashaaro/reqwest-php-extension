<?php

//$body = reqwest("https://httpbin.org/get");
//var_dump('body in php', $body);
$body = reqwest("http://localhost:1444/sleep");

var_dump('body in php 2', $body);

// $body = reqwest("https://http2.io/");

