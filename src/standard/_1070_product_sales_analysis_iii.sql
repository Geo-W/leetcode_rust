select t1.product_id, t1.year as first_year, t1.quantity, t1.price
from Sales t1
         left join (select product_id, min(year) as year
                    from Sales
                    group by product_id) t2 on t1.product_id = t2.product_id and t1.year = t2.year
where t2.year is not null