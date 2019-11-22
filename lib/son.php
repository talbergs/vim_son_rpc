#!/bin/php
<?php
/**
 * This script converts self-invented "short-object-notation" syntax into JSON.
 * Lines that begin with "#" are comment lines (only leading space is allowed).
 * Indentation can be any of user choosing, but is must be consistent though-out.
 * JSON will have only these data types: object, string, array, boolean (only true).
 * - object fields are grouped by indentation.
 * - key is separated from value by ":" (only one such symbol is allowed per line, else whole line is ignored).
 * - value following the key is trimmed. If spaces are used, it becomes of array type.
 * - key without value becomes `true`
 * - key followed by indented object will contain the objects value (if this key was declared "in pair"
 *   with string or array representation, that will be discarded while parsing).
 * - Duplicate keys in object will be "shadowed" (the last value will be used).
 *
 * Example input:
 *    |key1:
 *    |    sub_true:
 *    |    sub_obj:
 *    |        sub_arr: 1 2
 *    |key2: string
 *
 * Will produce this JSON:
 *    |{"key1": {"sub_true": true, "sub_obj": {"sub_arr": ["1", "2"]}}, "key2": "string"}
 *
 **
 *** The reason for script is to save keystrokes - braces, double quotes, meanwhile allowing comments
 *** for quick API interactions.
 **
 * Flags:
 * --pretty-print | -p       Pretty prints JSON
 *
 * 2019-08 MIT
 */

$out = [];

$pflag = 0;
$expect = 0;
$indent = 0;
$n = 0;
$rs = [&$out];

while ($argc-- > 1) switch($argv[$argc]) {
case '--help':case '-h': echo implode('',array_slice(file($argv[0]),2,32));exit(0);
case '--pretty-print':case '-p': $pflag = JSON_PRETTY_PRINT;break;
default: err('Unknown option');
}

// Yes, it is code-golfed, but machine readable.
$lines = explode(PHP_EOL, PHP_EOL.file_get_contents('php://stdin'));
while ($line = next($lines)) {
    if (!$rs) err('Erratic indentation error.', 2);

    $len = strlen($line);
    if (!$len) continue;
    $line = ltrim($line);

    $kv = explode(":", $line);
    if (count($kv) != 2) continue;

    list($k, $v) = $kv;
    if ($k[0] == '#') continue;

    end($rs);
    $indent = $len - strlen($line);
    ($indent && !$n && ($n = $indent));
    if ($expect == $indent) {
        $rs[key($rs)][$k] = to_val(trim($v));
        end($rs[key($rs)]);
    } else if ($indent - $n == $expect) {
        $lrs =& $rs[key($rs)];
        $lrs[key($lrs)] && ($lrs[key($lrs)] = []);
        $rs[] =& $lrs[key($lrs)];
        prev($lines) && ($expect = $indent);
    } else while ($expect != $indent && prev($lines)) array_pop($rs) && ($expect -= $n);
}

function err($s, $c = 1) {fwrite(STDERR, $s) && exit($c);}
function to_val($val) {
    if (!$val) return true;
    else if (strpos($val, ' ')) return explode(' ', $val);
    else return $val;
};

echo json_encode($out, $pflag);
