print('Start #################################################################');

db = db.getSiblingDB('zelda');
db.createUser(
    {
        user: 'zelda',
        pwd: 'zelda',
        roles: [{ role: 'dbOwner', db: 'zelda' }],
    },
);

print('END #################################################################');
