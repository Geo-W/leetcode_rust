DECLARE @total INT
set @total = (select count(*)
              from Seat)
select case
           when @total % 2 <> 0 and
                id = @total then id
           when id % 2 <> 0 then id + 1
           else id - 1 end
           as id,
       student
from Seat
order by id