DELETE
FROM Person
WHERE id in (SELECT id
             FROM (SELECT id, row_number() over (partition by email order by id ) as rn FROM Person) a
             WHERE a.rn > 1)