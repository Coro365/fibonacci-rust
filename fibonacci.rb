puts("please number!")
request = gets.strip.to_i
fibo = []

request.times do |i|
  case i
  when 0
    fibo.push(1)
  when 1
    fibo.push(1)
  else
    fibo.push(fibo[i - 1] + fibo[i - 2])
  end
end

pp fibo
