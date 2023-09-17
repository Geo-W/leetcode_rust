SELECT Department.name as Department, Employee.name as Employee, Employee.salary as Salary
FROM Employee
         LEFT JOIN Department on Employee.departmentId = Department.id
WHERE Employee.salary = (SELECT MAX(salary)
                         FROM Employee AS e
                         WHERE e.departmentId = Employee.departmentId)
