<?php

$test = ['testing' => 'test', 'testing', 'another', 23 => 'cool', 'hey'];

$test['testing'] = 'cool';

echo $test['testing'][0];