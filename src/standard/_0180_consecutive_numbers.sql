WITH t (id, num, rn) AS (select id, num, row_number() over (order by id) as rn from Logs)
select distinct t.num as ConsecutiveNums
from t
         left join t t2 on t.rn + 1 = t2.rn
         left join t t3 on t.rn + 2 = t3.rn
where t.num = t2.num
  and t.num = t3.num