-- @block Test of generic query
SELECT
    username,
    signature
FROM
    users
WHERE
    id = 1;

-- @block Querying all the accounts of user. Here get all accounts of nils
SELECT
    CONCAT (account.in_game_name, '#', account.tag) "name",
    account.id
FROM
    account
    INNER JOIN user_account ON account.id = user_account.account_id
WHERE
    user_account.user_id = 1;