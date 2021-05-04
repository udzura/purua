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
   ret = "Hello returned MyStr"
   return ret
end

println(getstr())
