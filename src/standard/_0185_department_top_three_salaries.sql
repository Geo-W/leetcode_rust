SELECT Department.name as Department, Employee.name as Employee, Employee.salary as Salary
FROM Employee
         LEFT JOIN Department on Employee.departmentId = Department.id
WHERE Employee.salary in (SELECT distinct top (3) salary
                          FROM Employee AS e
                          WHERE e.departmentId = Employee.departmentId
                          order by salary desc)
