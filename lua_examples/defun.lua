function myfunc()
   print("Call my own func!\n")
end

myfunc()

function println(myarg)
   print(myarg)
   print("\n")
end

println("Hello With LF")

function getstr()
   print("a")
   return "Hello returned MyStr"
end

println(getstr())
