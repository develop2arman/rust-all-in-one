
echo "Enter your message"
read message
#echo 'Enter the name of the branch:'
#read branch
# git add --all

echo "git added all"

git commit -a "${message}"

#if [ -n "$(git status - porcelain)" ];
#then
# echo "IT IS CLEAN"
#else
#

echo "Pushing data to remote server!!!"

git push -u origin main

echo "Pushed data successfully"

git status
