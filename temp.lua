--1

Number = {}
Number.new = function(self, n)
    self[1] = n
end
Number.distribute = function(self, external_sum)
    local self_sum = 0
    for i, number in ipairs(self) do
        self_sum = self_sum + number
    end
    return self_sum * external_sum
end

local a = Number
a:new(5)
print(a:distribute(7 + 10))
print(5 * 7 + 5 * 10)
print(a:distribute(7 * 10))
