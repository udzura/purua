function foobar()
    local a
    local b = 2
    local function f()
        local a = 1
        print(a)
        print(b)
    end
    print(a + b)
end