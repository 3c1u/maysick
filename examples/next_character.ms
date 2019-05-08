println('Give a capital letter... ');
let k = char_at('A' 0);
let r = (char_at(getchar() 0) + 1 - k) % 26;
print('The next letter is \'');
print(char_from(r + k));
println('\'.');

