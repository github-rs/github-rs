# Retrieve GitHub docs from the developer site for offline usage
from urllib.request import urlopen, urlretrieve
from urllib.parse import urlparse, urljoin
import os
import sys

BASE_URL = "https://developer.github.com/v3"
SECTIONS = [ "/"
           , "/media"
           , "/oauth"
           , "/oauth_authorizations"
           , "/auth"
           , "/troubleshooting"
           , "/pre-release"
           , "/previews"
           , "/versions"
           , "/activity"
           , "/activity/events"
           , "/activity/events/types"
           , "/activity/feeds"
           , "/activity/notifications"
           , "/activity/starring"
           , "/activity/watching"
           , "/gists"
           , "/gists/comments"
           , "/git"
           , "/git/blobs"
           , "/git/commits"
           , "/git/refs"
           , "/git/tags"
           , "/git/trees"
           , "/integrations"
           , "/integrations/installations"
           , "/issues"
           , "/issues/assignees"
           , "/issues/comments"
           , "/issues/events"
           , "/issues/labels"
           , "/issues/milestones"
           , "/issues/timeline"
           , "/migration"
           , "/migration/migrations"
           , "/migration/source_imports"
           , "/misc"
           , "/emojis"
           , "/gitignore"
           , "/licenses"
           , "/markdown"
           , "/meta"
           , "/rate_limit"
           , "/orgs"
           , "/orgs/members"
           , "/orgs/outside_collaborators"
           , "/orgs/teams"
           , "/orgs/hooks"
           , "/orgs/blocking"
           , "/projects"
           , "/projects/cards"
           , "/projects/columns"
           , "/pulls"
           , "/pulls/reviews"
           , "/pulls/comments"
           , "/pulls/review_requests"
           , "/reactions"
           , "/repos"
           , "/repos/branches"
           , "/repos/collaborators"
           , "/repos/comments"
           , "/repos/community"
           , "/repos/commits"
           , "/repos/contents"
           , "/repos/keys"
           , "/repos/deployments"
           , "/repos/downloads"
           , "/repos/forks"
           , "/repos/invitations"
           , "/repos/merging"
           , "/repos/pages"
           , "/repos/releases"
           , "/repos/statistics"
           , "/repos/statuses"
           , "/repos/traffic"
           , "/repos/hooks"
           , "/search"
           , "/users"
           , "/users/emails"
           , "/users/followers"
           , "/users/keys"
           , "/users/gpg_keys"
           , "/users/blocking"
           , "/enterprise"
           , "/enterprise/admin_stats"
           , "/enterprise/ldap"
           , "/enterprise/license"
           , "/enterprise/management_console"
           , "/enterprise/pre_receive_environments"
           , "/enterprise/pre_receive_hooks"
           , "/enterprise/search_indexing"
           , "/enterprise/orgs"
           ]

# Make all of our urls so we can extract the docs for them
url_list = list(map(lambda x: BASE_URL + x, SECTIONS))

out_folder="./"

num = 1
length = len(url_list)

for i in url_list:
    # Download and write html
    u = urlopen(i)
    file_name = i.split('v3')[1].replace('/', '-')[1:] + ".html"
    if len(file_name) <= 5:
        file_name = "index.html"
    meta = u.info()
    file_size = int(meta.get("Content-Length")[0])
    print("Downloading: %s" % file_name)
    file_size_dl = 0
    block_sz = 8192

    print("[ %d / %d ]" % (num, length))
    num += 1

    f = open(file_name, 'wb')
    while True:
        buffer = u.read(block_sz)
        if not buffer:
            break
        f.write(buffer)
    f.close()
