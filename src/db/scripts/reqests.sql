-- все стойки помещения
select *
from room2rack
         left join rack on room2rack.rack_id = rack.id
         left join point p on rack.leftAngle = p.id
         left join direction d on d.id = rack.hotend
where
    room2rack.room_id = TODO
;

-- все сенсоры принадлежащие этому помещению
select *
from sens_owners
         left join sensor on sens_owners.sens_id = sensor.id
         left join point on sensor.position = point.id
         left join triggers on sensor.trig = triggers.id
where
    sens_owners.room_id = TODO
  and sens_owners.rack_id IS NULL
;

-- все сенсоры принадлежаще стойке
select *
from sens_owners
         left join sensor on sens_owners.sens_id = sensor.id
         left join point on sensor.position = point.id
         left join triggers on sensor.trig = triggers.id
where
    sens_owners.room_id = TODO
  and sens_owners.rack_id = TODO
;

