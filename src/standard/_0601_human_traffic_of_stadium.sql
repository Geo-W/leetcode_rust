with TA as (select t1.id         as t1_id,
                   t1.visit_date as t1_visit_date,
                   t1.people     as t1_people,
                   t2.id         as t2_id,
                   t2.visit_date as t2_visit_date,
                   t2.people     as t2_people,
                   t3.id         as t3_id,
                   t3.visit_date as t3_visit_date,
                   t3.people     as t3_people
            from Stadium t1
                     left join Stadium t2 on t1.id + 1 = t2.id and t2.people >= 100
                     left join Stadium t3 on t1.id + 2 = t3.id and t3.people >= 100
            where t1.people >= 100
              and t2.id is not null
              and t3.id is not null)
select TA.t1_id as id, TA.t1_visit_date as visit_date, TA.t1_people as people
from TA
union
select TA.t2_id as id, TA.t2_visit_date as visit_date, TA.t2_people as people
from TA
union
select TA.t3_id as id, TA.t3_visit_date as visit_date, TA.t3_people as people
from TA
