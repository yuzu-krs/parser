######################################
# 入力正数の平方根を求めるプログラム #
# Written by Yasuharu Mizutani #
######################################
var C; #入力値を格納する変数

print("正の数を入力してください: ");
read(C);

# 漸化式を初項から計算していく．
# 20 回程度繰り返せば十分収束するでしょう．
var f := C; # 計算結果を格納する変数
repeat 20 f:=(f+C/f)/2.0;

print(C, "の平方根は "); print(f); println("です");
println(); #空行の挿入

print("ちなみに，");
print("sqrt(", C, ") の値は");
println(@sqrt(C), "です");
println(); #空行の挿入

println("誤差は ", f-@sqrt(C), "です");
