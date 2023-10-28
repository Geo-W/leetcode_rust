select Email
from Person
group by email
having count(Email) <> 1