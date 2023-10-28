with t(salary, rn) AS (select salary, DENSE_RANK() over (order by salary desc) as rn FROM Employee),
     sr(r) AS (SELECT 2)
select distinct salary as SecondHighestSalary
FROM sr
         LEFT JOIN t ON sr.r = t.rn