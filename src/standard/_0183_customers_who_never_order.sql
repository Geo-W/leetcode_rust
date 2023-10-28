select Customers.Name as Customers
from Customers
         left join Orders on Customers.id = Orders.CustomerId
where Orders.CustomerId is null