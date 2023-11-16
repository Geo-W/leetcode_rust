select name
from Employee
         INNER JOIN (select managerId
                     from Employee
                     group by managerId
                     having count(managerId) >= 5) b
                    on Employee.id = b.managerId