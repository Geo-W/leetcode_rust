select round(sum(tiv_2016), 2) as tiv_2016
from Insurance
         left join (select tiv_2015, count(*) as cnt from Insurance group by tiv_2015 having count(*) > 1) t2
                   on Insurance.tiv_2015 = t2.tiv_2015
         left join (select lat, lon, count(*) as cnt from Insurance group by lat, lon having count(*) = 1) t3
                   on Insurance.lat = t3.lat and Insurance.lon = t3.lon
where t2.tiv_2015 is not null
  and t3.lon is not null
