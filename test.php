<?php

$body = reqwest("http://httpbin.org/get");
var_dump('body in php', $body);
