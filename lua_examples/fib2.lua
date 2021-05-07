function dofib(n)
   if n < 2 then
      return 1
   else
      return dofib(n-1) + dofib(n-2)
   end
   print("Unreachable!\n")
end

print(dofib(20))
print("\n")
